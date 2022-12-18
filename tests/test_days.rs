use anyhow::Result;
use std::path::Path;

fn run_day<A, B>(day: usize, f: fn(&Path) -> Result<(A, Option<B>)>) -> Result<(A, Option<B>)> {
    f(format!("data/day{}.txt", day).as_ref())
}

#[test]
fn test_day1() -> Result<()> {
    assert_eq!(
        run_day(1, advent_of_code_2022::day1::main)?,
        (71506, Some(209603))
    );
    Ok(())
}

#[test]
fn test_day2() -> Result<()> {
    assert_eq!(
        run_day(2, advent_of_code_2022::day2::main)?,
        (15523, Some(15702))
    );
    Ok(())
}

#[test]
fn test_day3() -> Result<()> {
    assert_eq!(
        run_day(3, advent_of_code_2022::day3::main)?,
        (8401, Some(2641))
    );
    Ok(())
}

#[test]
fn test_day4() -> Result<()> {
    assert_eq!(
        run_day(4, advent_of_code_2022::day4::main)?,
        (582, Some(893))
    );
    Ok(())
}

#[test]
fn test_day5() -> Result<()> {
    assert_eq!(
        run_day(5, advent_of_code_2022::day5::main)?,
        ("TLNGFGMFN".to_owned(), Some("FGLQJCMBD".to_owned()))
    );
    Ok(())
}

#[test]
fn test_day6() -> Result<()> {
    assert_eq!(
        run_day(6, advent_of_code_2022::day6::main)?,
        (1794, Some(2851))
    );
    Ok(())
}

#[test]
fn test_day7() -> Result<()> {
    assert_eq!(
        run_day(7, advent_of_code_2022::day7::main)?,
        (1_428_881, Some(10_475_598))
    );
    Ok(())
}

#[test]
fn test_day8() -> Result<()> {
    assert_eq!(
        run_day(8, advent_of_code_2022::day8::main)?,
        (1812, Some(315_495))
    );
    Ok(())
}

#[test]
fn test_day9() -> Result<()> {
    assert_eq!(
        run_day(9, advent_of_code_2022::day9::main)?,
        (6357, Some(2627))
    );
    Ok(())
}

#[test]
fn test_day10() -> Result<()> {
    assert_eq!(
        run_day(10, advent_of_code_2022::day10::main)?,
        (
            12540,
            Some(
                [
                    "#### ####  ##  #### #### #    #  # #### ",
                    "#    #    #  #    # #    #    #  # #    ",
                    "###  ###  #      #  ###  #    #### ###  ",
                    "#    #    #     #   #    #    #  # #    ",
                    "#    #    #  # #    #    #    #  # #    ",
                    "#    ####  ##  #### #### #### #  # #### ",
                ]
                .join("\n")
            )
        )
    );
    Ok(())
}

#[test]
fn test_day11() -> Result<()> {
    assert_eq!(
        run_day(11, advent_of_code_2022::day11::main)?,
        (119_715, Some(18_085_004_878))
    );
    Ok(())
}

#[test]
fn test_day12() -> Result<()> {
    assert_eq!(
        run_day(12, advent_of_code_2022::day12::main)?,
        (481, Some(480))
    );
    Ok(())
}

#[test]
fn test_day13() -> Result<()> {
    assert_eq!(
        run_day(13, advent_of_code_2022::day13::main)?,
        (6101, Some(21909))
    );
    Ok(())
}

#[test]
fn test_day14() -> Result<()> {
    assert_eq!(
        run_day(14, advent_of_code_2022::day14::main)?,
        (683, Some(28_821))
    );
    Ok(())
}

#[test]
fn test_day15() -> Result<()> {
    assert_eq!(
        run_day(15, advent_of_code_2022::day15::main)?,
        (4_665_948, Some(13_543_690_671_045))
    );
    Ok(())
}

// Needs to be ignored because my solution is slow :(
#[test]
#[ignore]
fn test_day16() -> Result<()> {
    assert_eq!(
        run_day(16, advent_of_code_2022::day16::main)?,
        (2056, Some(2513))
    );
    Ok(())
}

#[test]
fn test_day17() -> Result<()> {
    assert_eq!(run_day(17, advent_of_code_2022::day17::main)?, (3175, None));
    Ok(())
}

#[test]
fn test_day18() -> Result<()> {
    assert_eq!(
        run_day(18, advent_of_code_2022::day18::main)?,
        (4548, Some(2588))
    );
    Ok(())
}
