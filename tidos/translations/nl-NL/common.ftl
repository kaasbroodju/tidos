greeting = Hallo
farewell = Dag
shared-photos =
    {$userName} {$photoCount ->
    [one] heeft een nieuwe foto toegevoegd
    *[other] heeft {$photoCount} nieuwe foto's toegevoegd
        } aan {$userGender ->
    [male] zijn stream
    [female] haar stream
    *[other] hun stream
        }.
