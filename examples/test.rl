
skillset S {

    resource R {
        state { On Off }
        initial Off
        transition all
    }

    event E {
        guard R==Off and not R==Off
        effect R -> On
    }
}