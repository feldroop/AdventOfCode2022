use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{char, digit1, line_ending, space1},
        streaming::not_line_ending,
    },
    multi::many0,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, Parser,
};

use crate::filesystem_model::{Command, FilesystemItem, RelativeDirectory};

pub fn parse(input: &str) -> Vec<Command> {
    let command = alt((cd_command, ls_command));
    let (rest, commands) = many0(command)(input).expect("Expected a valid command sequence");

    assert!(rest.is_empty());

    commands
}

fn cd_command(input: &str) -> IResult<&str, Command> {
    let command_prefix = pair(char('$'), space1);

    use RelativeDirectory::*;
    let parent = tag("..").map(|_| Parent);
    let root = char('/').map(|_| Root);
    let regular = not_line_ending.map(|name: &str| Child {
        name: name.to_owned(),
    });

    let cd_input = alt((parent, root, regular));

    let cd_command = separated_pair(tag("cd"), space1, cd_input)
        .map(|(_, cd_input)| Command::ChangeDirectory { target: cd_input });

    delimited(command_prefix, cd_command, line_ending)(input)
}

fn ls_command(input: &str) -> IResult<&str, Command> {
    let command_prefix = pair(char('$'), space1);

    let ls_command = tag("ls");
    let ls_line = delimited(command_prefix, ls_command, line_ending);

    let directory_tag = pair(tag("dir"), space1);
    let directory =
        preceded(directory_tag, not_line_ending).map(|dir_name: &str| FilesystemItem::Directory {
            name: dir_name.to_owned(),
        });

    let file =
        separated_pair(digit1::<&str, _>, space1, not_line_ending).map(|(file_size, file_name)| {
            FilesystemItem::File {
                name: file_name.to_owned(),
                size: file_size
                    .parse()
                    .expect("This should be a sequence of digits"),
            }
        });

    let ls_output_line = terminated(alt((directory, file)), line_ending);
    let full_ls_output = many0(ls_output_line);

    pair(ls_line, full_ls_output)
        .map(|(_, output_items)| Command::List { output_items })
        .parse(input)
}
