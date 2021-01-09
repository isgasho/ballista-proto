description = "Ballista protocol buffer format"

plugins {
    java
    id("com.google.protobuf") version "0.8.11"
    id("idea")
}

repositories {
    mavenLocal()
    mavenCentral()
    jcenter()
}

group = "org.ballistacompute"
version = "0.1.0"

sourceSets {
    main {
        proto {
            srcDir("../../proto")
        }
    }
}

dependencies {
    implementation("com.google.protobuf:protobuf-java:3.11.4")
}
