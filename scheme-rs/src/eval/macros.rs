use std::borrow::Borrow;
use std::rc::Rc;

use crate::environment::Env;
use crate::error::{LispError, LispResult};
use crate::lisp_val::LispVal;

use super::util::get_tails;

// {- loadLocal - Determine if pattern matches input, loading input into pattern variables as we go,
// in preparation for macro transformation. -}
// loadLocal :: [Env] -> Env -> Env -> Env -> Env -> LispVal -> LispVal -> LispVal -> Int -> [Int] -> [(Bool, Bool)] -> String -> IOThrowsError LispVal
// loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers pattern input ellipsisLevel ellipsisIndex listFlags esym = do
//     --case (trace ("loadLocal [" ++ (show pattern) ++ "] [" ++ (show input) ++ "] flags = " ++ (show listFlags) ++ " ...lvl = " ++ (show ellipsisLevel) ++ " ...indx = " ++ (show ellipsisIndex)) (pattern, input)) of
//     case (pattern, input) of

//         ((DottedList ps p), (DottedList isRaw iRaw)) -> do

//             -- Split input into two sections:
//             --   is - required inputs that must be present
//             --   i  - variable length inputs to each compare against p
//             let isSplit = splitAt (length ps) isRaw
//             let is = fst isSplit
//             let i = (snd isSplit) ++ [iRaw]

//             result <- loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers (List ps) (List is) ellipsisLevel ellipsisIndex listFlags esym
//             case result of
//             Bool True -> --  By matching on an elipsis we force the code
//                             --  to match p against all elements in i.
//                             loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers
//                                     (List $ [p, Atom esym])
//                                     (List i)
//                                     ellipsisLevel -- Incremented in the list/list match below
//                                     ellipsisIndex
//                                     (flagDottedLists listFlags (True, True) $ length ellipsisIndex) -- Do not think we need to flag ... that are passed over, since this is a direct comparison of both cdr's
//                                     esym
//             _ -> return $ Bool False

//         (List (p : ps), List (i : is)) -> do -- check first input against first pattern, recurse...

//             let nextHasEllipsis = macroElementMatchesMany pattern esym
//             let level = if nextHasEllipsis then ellipsisLevel + 1
//                                         else ellipsisLevel
//             let idx = if nextHasEllipsis
//                         then if (length ellipsisIndex == level)
//                                 -- This is not the first match, increment existing index
//                                 then do
//                                 let l = splitAt (level - 1) ellipsisIndex
//                                 (fst l) ++ [(head (snd l)) + 1]
//                                 -- First input element that matches pattern; start at 0
//                                 else ellipsisIndex ++ [0]
//                         else ellipsisIndex

//             -- At this point we know if the input is part of an ellipsis, so set the level accordingly
//             status <- checkLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers level idx p i listFlags esym
//             case status of
//                 -- No match
//                 Bool False -> if nextHasEllipsis
//                                 {- No match, must be finished with ...
//                                 Move past it, but keep the same input. -}
//                                 then do
//                                         case ps of
//                                             [Atom _] -> return $ Bool True -- An otherwise empty list, so just let the caller know match is done
//                                             _ -> loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers (List $ tail ps) (List (i : is)) ellipsisLevel ellipsisIndex listFlags esym
//                                 else return $ Bool False
//                 -- There was a match
//                 _ -> if nextHasEllipsis
//                         then
//                             loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers pattern (List is)
//                             ellipsisLevel -- Do not increment level, just wait until the next go-round when it will be incremented above
//                             idx -- Must keep index since it is incremented each time
//                             listFlags
//                             esym
//                         else loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers (List ps) (List is) ellipsisLevel ellipsisIndex listFlags esym

//         -- Base case - All data processed
//         (List [], List []) -> return $ Bool True

//         -- Ran out of input to process
//         (List (_ : _), List []) -> do
//             if (macroElementMatchesMany pattern esym)
//             then do
//                 -- Ensure any patterns that are not present in the input still
//                 -- have their variables initialized so they are ready during transformation
//                 -- Note:
//                 -- Appending to eIndex to compensate for fact we are outside the list containing the nary match
//                 let flags = getListFlags (ellipsisIndex ++ [0]) listFlags
//                 flagUnmatchedVars defEnv outerEnv localEnv identifiers pattern (fst flags) esym
//             else return $ Bool False

//         -- Pattern ran out, but there is still input. No match.
//         (List [], _) -> return $ Bool False

//         -- Check input against pattern (both should be single var)
//         (_, _) -> checkLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers ellipsisLevel ellipsisIndex pattern input listFlags esym

fn check_pattern(ps: &[LispVal], is: Vec<LispVal>, flag: bool, ellipsis_symbol: String,) -> LispResult<LispVal> {
    if let ([LispVal::DottedList(ds, d), _]) = ps {
        match is {
            [LispVal::DottedList(_, _), ref xs @ .. ] => {
                load_local(def_env,outer_env,divert_env,local_env,rename_env,identifiers ,
                                  LispVal::List(vec![ds ++ [d, Atom esym]]),
                                  LispVal::List(is), 0, vec![],
                                  (flagDottedLists [] (False, False) $ 0 + (length $ filter (filter_esym(ellipsis_symbol)) ds)),
                                  esym
            }
        }
    } else {
        // load_local(def_env, outer_env, divert_env, local_env, rename_env, identifiers, LispVal::List(ps), LispVal::List(is), 0, [],[], ellipsis_symbol)

        todo!()
    }
    // return transform_rule(def_env, outer_env, divert_env, local_env, rename_env, cleanup_env, dim, identifiers, ellipsis_symbol, 0, [], (List []), template)
}
// -- A pair at the outmost level must be transformed to use the ellipsis,
// -- or else its nary match will not work properly during pattern matching.
// checkPattern ps@(DottedList ds d : _) is True = do
//     case is of
//     (DottedList _ _ : _) -> do
//         loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers
//                                 (List $ ds ++ [d, Atom esym])
//                                 (List is)
//                                 0 []
//                                 (flagDottedLists [] (False, False) $ 0 + (length $ filter (filterEsym esym) ds)) -- Mark any ellipsis we are passing over
//                                 esym
//     (List _ : _) -> do
//         loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers
//                                 (List $ ds ++ [d, Atom esym])
//                                 (List is)
//                                 0 []
//                                 (flagDottedLists [] (True, False) $ 0 + (length $ filter (filterEsym esym) ds)) -- Mark any ellipsis we are passing over
//                                 esym
//     _ -> loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers (List ps) (List is) 0 [] [] esym

// -- No pair, immediately begin matching
// checkPattern ps is _ = loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers (List ps) (List is) 0 [] [] esym

fn match_rule(
    def_env: Vec<Env>,
    env: Env,
    divert_env: Env,
    dim: bool,
    identifiers: LispVal,
    local_env: Env,
    rename_env: Env,
    cleanup_env: Env,
    rule: LispVal,
    input: LispVal,
    ellipsis_symbol: String,
) -> LispResult<LispVal> {
    if let LispVal::List(xs) = &rule {
        if let [pattern, template] = &xs[..] {
            if let LispVal::List(input_var) = input {
                let is = get_tails(&input_var)?;
                let p = match pattern {
                    LispVal::DottedList(ds, d) => {
                        if let [LispVal::Atom(l), ref ls @ ..] = &ds[..] {
                            (
                                &LispVal::List(Rc::new(vec![
                                    LispVal::Atom(l.to_string()),
                                    LispVal::DottedList(Rc::new(ls.to_vec()), d.clone()),
                                ])),
                                true,
                            )
                        } else {
                            (pattern, false)
                        }
                    }
                    _ => (pattern, false),
                };
                if let (LispVal::List(xs), flag) = p {
                    if let [LispVal::Atom(_), ref ls @ ..] = &xs[..] {
                        if let LispVal::Bool(false) = check_pattern(&xs[..], is, flag)? {
                            return Ok(LispVal::Nil);
                        } else {
                            todo!()
                            // return transform_rule(def_env, outer_env, divert_env, local_env, rename_env, cleanup_env, dim, identifiers, ellipsis_symbol, 0, [], (List []), template)
                        }
                    }
                }
                return Err(LispError::BadSpecialForm(
                    "Malformed rule in syntax-rules".to_string(),
                    LispVal::String(format!("{:?}", p)),
                ));
            }
        }
    }
    Err(LispError::BadSpecialForm(
        "Malformed rule in syntax-rules".to_string(),
        LispVal::List(Rc::new(vec![
            LispVal::Atom("rule: ".to_string()),
            rule,
            LispVal::Atom("input: ".to_string()),
            input,
        ])),
    ))
}

//  where
//    -- A pair at the outmost level must be transformed to use the ellipsis,
//    -- or else its nary match will not work properly during pattern matching.
//    checkPattern ps@(DottedList ds d : _) is True = do
//      case is of
//        (DottedList _ _ : _) -> do
//          loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers
//                                   (List $ ds ++ [d, Atom esym])
//                                   (List is)
//                                    0 []
//                                   (flagDottedLists [] (False, False) $ 0 + (length $ filter (filterEsym esym) ds)) -- Mark any ellipsis we are passing over
//                                   esym
//        (List _ : _) -> do
//          loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers
//                                   (List $ ds ++ [d, Atom esym])
//                                   (List is)
//                                    0 []
//                                   (flagDottedLists [] (True, False) $ 0 + (length $ filter (filterEsym esym) ds)) -- Mark any ellipsis we are passing over
//                                   esym
//        _ -> loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers (List ps) (List is) 0 [] [] esym

//    -- No pair, immediately begin matching
//    checkPattern ps is _ = loadLocal defEnv outerEnv divertEnv localEnv renameEnv identifiers (List ps) (List is) 0 [] [] esym

fn macro_transform<F>(
    def_env: Vec<Env>,
    env: Env,
    divert_env: Env,
    rename_env: Env,
    cleanup_env: Env,
    dim: bool,
    identifiers: LispVal,
    rules: Vec<LispVal>,
    input: LispVal,
    apply: F,
    ellipsis_symbol: String,
) -> LispResult<LispVal>
where
    F: Fn(LispVal, LispVal, Vec<LispVal>) -> LispResult<LispVal>,
{
    for rule in rules {
        let result = match_rule(
            def_env,
            env,
            divert_env,
            dim,
            identifiers,
            local_env,
            rename_env,
            cleanup_env,
            rule,
            input,
            ellipsis_symbol,
        );
    }
}
// macroTransform defEnv env divertEnv renameEnv cleanupEnv dim identifiers (rule@(List _) : rs) input apply esym = do
//   localEnv <- liftIO $ nullEnv -- Local environment used just for this invocation
//                                -- to hold pattern variables
//   result <- matchRule defEnv env divertEnv dim identifiers localEnv renameEnv cleanupEnv rule input esym
//   case result of
//     -- No match, check the next rule
//     Nil _ -> macroTransform defEnv env divertEnv renameEnv cleanupEnv dim identifiers rs input apply esym
//     _ -> do
//         -- Walk the resulting code, performing the Clinger algorithm's 4 components
//         walkExpanded defEnv env divertEnv renameEnv cleanupEnv dim True False (List []) result apply

pub fn macro_eval(env: &Env, val: &LispVal) -> LispResult<LispVal> {
    if let v @ LispVal::List(xs) = val {
        if let [LispVal::Atom(var), ref xs @ ..] = &xs[..] {
            if let Some(LispVal::Syntax(ref syntax)) = env.lookup(var) {
                let rename_env = Env::new();
                let cleanup_env = Env::new();
                // let expanded = macro_transform

                // -- Transform the input and then call macroEval again,
                // -- since a macro may be contained within...
                // expanded <- macroTransform [defEnv] env env renameEnv cleanupEnv
                //                             definedInMacro
                //                             (List identifiers) rules lisp apply
                //                             ellipsis
                // _macroEval env expanded apply
                // -- Useful debug to see all exp's:
                // -- macroEval env (trace ("exp = " ++ show expanded) expanded)
            }
        }
    }
    LispResult::Ok(val.clone())
    // Do macro transformations (as necessary)
    // This will produce a LispVal which we'll then eval
}
