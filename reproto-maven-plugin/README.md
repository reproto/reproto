# reproto-maven-plugin

This is a maven plugin intended to make it simple to integrate reproto into the lifecycle
of a maven project.

You can run examples with the provided [`run-examples.sh`](run-examples.sh) script.

## Usage

```
<project>
  ...
  <build>
    <plugins>
      <plugin>
        <groupId>se.tedro.maven.plugins</groupId>
        <artifactId>reproto-maven-plugin</artifactId>
        <version>0.0.1-SNAPSHOT</version>
        <extensions>true</extensions>

        <configuration>
          <targets>
            <target>heroic.v1</target>
          </targets>
        </configuration>
      </plugin>
      ...
    </plugins>
    ...
  </build>
  ...
</project>
```
