#Author: samundeswari.kothandaraman@travelport.com
#Keywords Summary :
#Feature: List of scenarios.
#Scenario: Business rule through list of steps with arguments.
#Given: Some precondition step
#When: Some key actions
#Then: To observe outcomes or validation
#And,But: To enumerate more Given,When,Then steps
#Scenario Outline: List of steps for data-driven as an Examples and <placeholder>
#Examples: Container for s table
#Background: List of steps run before each of the scenarios
#""" (Doc Strings)
#| (Data Tables)
#@ (Tags/Labels):To group Scenarios
#<> (placeholder)
#""
## (Comments)
#Sample Feature Definition Template
@tag
Feature: TC07_Shop with RoundTrip,Price, Book with multi PAX, Ticket, AMOA, SeatMap , AMF in UAPI and MANUAL EVEN EXCHANGE  on smartpoint_1G


Scenario Outline: Post ticketing - ManualEvenExchange
    Given Launch Smartpoint
    #And Change Application settings to <Core>
    #And Login Smartpoint
    #Given Change the currency specifics to <country>
    And Change the currency specific to <country>
    And Emulate the <PCC>
    #And Retrieve the ticketed <PNR>
    And Retrieve the ticketed PNR
    #When Change the <Class> of travel for <Segment>
    When Change the <Date> of travel for <Segment>
    And Cancel Fares and add new Fares for the PNR
		And Validate Tickets status for ticketed PNR
		And perform Manual Even exchange
		Then Validate Ticket status
		And Close SmartPoint


    Examples: 
      | country | PCC   |      Tab        |Segment |  Date   | Class |
      | DEN     | 2O9C  | VoluntaryChange |   2	   |  30AUG  |   y   |
