Feature: TC05_Shop with RoundTrip, Price, Book, Ticket, AMOA, SeatMap & AMF in UAPI and ETICKET EXCHANGE (ACM) on smartpoint_1G
Scenario Outline: Post ticketing - Voluntary Exchange ACM Exchange flow
    Given Launch Smartpoint
    #And Change Application settings to <Core>
    #And Login Smartpoint
    #And Change the currency specific to <country>
    #And Emulate the <PCC>
    And Retrieve the ticketed PNR
    When Change the <Class> of travel for <Segment>
    And Cancel Fares and add new Fares for the PNR
    And Launch SAT
    And Navigate to SAT <Tab>
    And Check the difference in Price
    And Perform Voluntary exchange Operations for <Customer>
    Then Validate Ticket status
    And Close SmartPoint

    Examples: 
      | country | PCC   |      Tab        |Segment |  Class   |      Customer     |
      | LON     | 259S  | VoluntaryChange |   1	   |    B     |  Delta Vacations  |