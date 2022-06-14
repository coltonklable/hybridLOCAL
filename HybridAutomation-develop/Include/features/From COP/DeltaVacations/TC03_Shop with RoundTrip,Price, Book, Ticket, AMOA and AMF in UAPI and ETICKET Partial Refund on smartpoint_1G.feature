@tag
Feature: TC03_Shop with RoundTrip,Price, Book, Ticket, AMOA and AMF in UAPI and ETICKET Partial Refund on smartpoint_1G
  
  @tag1
  Scenario Outline:  Post ticketing - Partial Refund flow
    Given Launch Smartpoint
    # And Change Application settings to <Core>
    #And Login Smartpoint
    And Change the currency specific to <country>
    And Emulate the <PCC>
    And Retrieve the ticketed PNR
    And Launch SAT
    And Navigate to SAT <Tab>
    And Validate the Retrieved ticket number and Status
    When Perform <Refund> Operation for <Customer>
    Then Validate Refunded Ticket status
    #And Cancel the Itinerary
    And Close SmartPoint

Examples:
    	| country | PCC  |      Tab       |      Refund     |     Customer    |
     	|   DEN   | 259S | TicketRefund   |  PARTIALREFUND  |	 DeltaVacations |