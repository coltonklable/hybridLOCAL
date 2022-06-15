Feature: TC06_Shop with RoundTrip,Price, Book, Ticket in UAPI and MANUAL FULL REFUND on smartpoint_1G
  
 Scenario Outline: Post ticketing - Manual Full Refund
   Given Launch Smartpoint
   #And Change Application settings to <Core>
   And Login Smartpoint
   #And Change the currency to the currency of PCC CITY 
   And Emulate the <PCC>
   And Retrieve the ticketed PNR
   And Validate Tickets status for ticketed PNR
   And Perform Manual Full Refund Operation <Date> for <Customer>  
   Then Validate Tickets status as RFND in *HTE Response
   And Cancel the Itinerary
   And Close SmartPoint


    Examples: 
      |     Core    | country | PCC  |   Date    |     Customer     |
      |  GalileoUAT | LON     | 259S |  25MAY22  | Delta Vacations  |
  