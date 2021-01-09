description = "Ballista protocol buffer format"

plugins {
    java
    `java-library`
    `maven-publish`

    id("com.google.protobuf") version "0.8.11"

    //TODO: this has to be uncommented when pushing a release to sonatype but commented out
    // for github actions to work ... would be nice to find a better solution for this
    //id("org.hibernate.build.maven-repo-auth") version "3.0.0"

    id("org.jetbrains.dokka") version "0.10.1"
    signing
}

repositories {
    mavenLocal()
    mavenCentral()
    jcenter()
}

group = "org.ballistacompute"
version = "0.1.0"

extra["isReleaseVersion"]  = !version.toString().endsWith("SNAPSHOT")

sourceSets {
    main {
        proto {
            srcDir("../../proto")
        }
    }
}

dependencies {
    implementation(gradleApi())
    implementation("com.google.protobuf:protobuf-java:3.11.4")
}

val implementation by configurations
val testImplementation by configurations

tasks.dokka {
    outputFormat = "html"
    outputDirectory = "$buildDir/javadoc"
}

//tasks.jar {
//    manifest {
//        attributes(
//            "Implementation-Title" to "${rootProject.name}-${archiveBaseName.get()}",
//            "Implementation-Version" to rootProject.version,
//            "Build-Timestamp" to java.time.Instant.now()
//        )
//    }
//}

val sourcesJar = tasks.create<Jar>("sourcesJar") {
    archiveClassifier.set("sources")
    from(sourceSets.getByName("main").allSource)
}

val javadocJar = tasks.create<Jar>("javadocJar") {
    archiveClassifier.set("javadoc")
    from("$buildDir/javadoc")
}

java {
    withJavadocJar()
}

//publishing {
//    repositories {
//        maven {
//            name = "sonatype"
//            url = uri("https://oss.sonatype.org/service/local/staging/deploy/maven2")
//            credentials {
//                username = System.getenv("SONATYPE_USERNAME")
//                password = System.getenv("SONATYPE_PASSWORD")
//            }
//        }
//    }
//
//    publications {
//        create<MavenPublication>("mavenKotlin") {
//            groupId = "org.ballistacompute"
//            version = rootProject.version as String?
//
//            pom {
//                name.set("Ballista Compute")
//                description.set("JVM query engine based on Apache Arrow")
//                url.set("https://github.com/ballista-compute/ballista")
//                licenses {
//                    license {
//                        name.set("The Apache License, Version 2.0")
//                        url.set("http://www.apache.org/licenses/LICENSE-2.0.txt")
//                    }
//                }
//                developers {
//                    developer {
//                        id.set("andygrove")
//                        name.set("Andy Grove")
//                        email.set("andygrove73@gmail.com")
//                    }
//                }
//                scm {
//                    connection.set("scm:git:git://github.com/ballista-compute/ballista-proto.git")
//                    developerConnection.set("scm:git:ssh://github.com/ballista-compute/ballista-proto.git")
//                    url.set("https://github.com/ballista-compute/ballista-proto/")
//                }
//            }
//
////            from(components["kotlin"])
//            artifact(sourcesJar)
//            artifact(javadocJar)
//        }
//    }
//}
//

//signing {
//    setRequired({
//        (project.extra["isReleaseVersion"] as Boolean) && gradle.taskGraph.hasTask("publish")
//    })
//    sign(publishing.publications["mavenKotlin"])
//}
