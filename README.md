# Fig

**Fig** offers one more alternative to manage your application's configuration files. It takes those configuration templates under version control and creates copies that can be customized to the target environment. This problem is solved in many ways, but this approach is certainly the easiest one.

## How Fig Works

**Fig** uses a configuration file to map all configuration template files in the project. The working directory where the command is called needs to contain the file `fig.json`, otherwise the app will ask the user if they want to create one. The paths in the `fig.json` must be relative to where it is located. Example:

```
{
  "fig": "0.1.0",
  "version": "1.0",
  "mappings": [
    {
      "source": "src/resources/application.properties-template",
      "target": "src/resources/application.properties",
      "profiles": [
        {
          "name": "dev",
          "description": "To be used for development"
          "entries": [
            {
              "api-server": "http://127.0.0.1:8080/api",
              "enable-secret": "a1a1a1a1a1a1a"
            }
          ]
        }
      ]
    }
  ]
}
```
In this case, `fig.json` is in the same folder as `src/`. If there is no secret information in your `fig.json` file, consider tracking it under version control so everybody in the team can benefit from it.

At this time, Fig supports `.properties` files only. This choice was made based on its first use case. We can certainly extended it to support other configuration formats if there is enough demmand.

When **Fig** runs for the first time, it goes through the `fig.json` mappings and creates all the target configuration files that don't exist yet. If the `.gitignore` file is present in the same working directory, then it asks the user if they want to add all mapped targets to it.

For the targets that already exists, **Fig** conpares each entry with their sources and in case of differences, it asks the user if they want to keep the same value or adopt the one from the source. The default behavior is to keep it from the target, which can be confirmed by simply pressing `Enter`. Entries that exists in the source but do not exist in the target are automatically added to the target, with **Fig** asking if the user wants to keep the value from the source or change it. The default behavior is to keep it from the source. For entries that only exists in the target, **Fig** asks the user if they want to keep or remove them. The default behavior is to keep them. **Fig** ignores all entries in the target that matches the ones in the source. This iterative process runs in the console and continues until all mappings are processed. Each target file is saved once all entries are checked.

Sometimes, the configuration is very similar in multiple environments. To avoid having to configure each environment manually, you can use profiles to customize a set of entries for those environments. It can also be used to temporarely configure the application in a certain way just to work in a particular feature and then go back to the original config later. The entries in the profile work like decisions the user already made, so **Fig** simply takes them into account in the target file without asking.

Changes in the target file can be restored to the previous values. **Fig** makes a backup of the target before changing it so the user has one chance to recover it.

## How To Use Fig

Navigate to the root of the repository, where you can find the file `.gitignore`:

    $ cd path/to/repo

Run **Fig** for the first time to create the `fig.json` file:

    $ fig

Run **Fig** again to take into account your changes in the `fig.json` file. You can also consider one of the profiles:

    $ fig --profile dev
