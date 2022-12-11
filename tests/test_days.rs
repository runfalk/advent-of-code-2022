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
