use std::{
    env::args,
};
mod utils; use utils::{
    mkdir,
    echo,
};

/*
半公式の deno-init コマンドを元に、デフォルト (VSCode モード) では

1. mkdir dirname && cd dirname
2. deno-init --yes
3. mkdir .vscode && cd .vscode
4. touch settings.json && export DENO_ENABLING_STEEINGS >> settings.json

というイメージだが、
- できれば最初から src ディレクトリ、src/main.ts も作って
  deno.json の "include" に "src/" を入れておきたい
  (**／*.ts でもよさそう (deno では設定用の .ts ファイルとかないので))
- deno-init では "importMap" が指定されない？　みたいなので指定する
- したがってさらに touch import_map.json && export 〜
*/

fn main() {
    /* candidates for options
    let operation_mode = "auto"; // <-> "interactive"
    let main_lang = "ts"; // <-> "js"
    let editor = "code"; // <-> "vim", "nvim", "emacs", "atom", ...
    */

    const VSCODE_SETTINGS_DEFAULT: &'static str =
"{
    \"deno.enable\": true,
    \"deno.lint\": true,
    \"deno.unstable\": true,
    \"deno.importMap\": \"./import_map.json\"
}";
    const DENO_SETTINGS_DEFAULT: &'static str =
"{
    \"compilerOptions\": {
        \"allowJs\": false,
        \"strict\": true
    },
    \"lint\": {
        \"files\": {
            \"include\": [\"src/\"]
        }
    },
    \"fmt\": {
        \"files\": {
            \"include\": [\"src/\"]
        },
        \"options\": {
            \"indentWidth\": 4,
            \"singleQuote\": false,
            \"useTabs\": false,
            \"proseWrap\": \"always\"
        }
    },
    \"importMap\": \"import_map.json\",
    \"tasks\": {
        \"dev\": \"deno run --alow-net=localhost:8080\"
    }
}";
    const IMPORT_MAP_SETTING_DEFAULT: &'static str =
"{
    \"imports\": {
        \"std/\": \"https://deno.land/std@0.141.0/\"
    }
}";


    // とりあえず options なしで実装する
    // $ deno-new dirname
    let mut args = args();
    let _command_executing_path = args.next();

    /* here processing options */

    let target_dir_path = {
        let arg = args.next();
        if arg.is_none() {
            println!("missing target dirname");
            return;
        }
        arg.unwrap()
    };

    let vscode_config_dir_path = format!("{}/.vscode", &target_dir_path);
    let src_dir_path = format!("{}/src", &target_dir_path);

    let vscode_config_file_path = format!("{}/settings.json", &vscode_config_dir_path);
    let main_src_file_path = format!("{}/main.ts", &src_dir_path);
    let deno_config_file_path = format!("{}/deno.json", &target_dir_path);
    let import_map_config_file_path = format!("{}/import_map.json", &target_dir_path);
    
    mkdir(target_dir_path);

    mkdir(vscode_config_dir_path);
    echo(VSCODE_SETTINGS_DEFAULT, vscode_config_file_path);

    mkdir(src_dir_path);
    echo("", main_src_file_path);

    echo(DENO_SETTINGS_DEFAULT, deno_config_file_path);
    echo(IMPORT_MAP_SETTING_DEFAULT, import_map_config_file_path);
}
