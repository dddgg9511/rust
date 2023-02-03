projectName=$1
basePath=~/desktop/work/jupiter-server-
gradlePath=$basePath$projectName

$gradlePath/gradlew bootRun --args='--spring.profiles.active=local' -p $gradlePath
