<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>EDreams_SeatMap</name>
   <tag></tag>
   <elementGuidId>e8c2ea1f-9542-4251-87d0-e6dc3a81f77a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://localhost:8080/kestrel/AirService</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:xsd=&quot;http://www.w3.org/2001/XMLSchema&quot; xmlns:xsi=&quot;http://www.w3.org/2001/XMLSchema-instance&quot; xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
   &lt;soap:Body>
      &lt;air:SeatMapReq TargetBranch=&quot;P7113930&quot; ReturnSeatPricing=&quot;true&quot; xmlns:common_v50_0=&quot;http://www.travelport.com/schema/common_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
         &lt;air:AirSegment Key=&quot;TLsU9ytY1DKAMGdHAAAAAA==&quot; Group=&quot;0&quot; Carrier=&quot;KL&quot; CabinClass=&quot;Economy&quot; FlightNumber=&quot;1673&quot; ProviderCode=&quot;1G&quot; Origin=&quot;AMS&quot; Destination=&quot;BCN&quot; DepartureTime=&quot;2021-12-10T14:25:00.000+01:00&quot; ArrivalTime=&quot;2021-12-10T16:35:00.000+01:00&quot; TravelTime=&quot;130&quot; Distance=&quot;771&quot; ClassOfService=&quot;G&quot; ETicketability=&quot;Yes&quot; Equipment=&quot;73H&quot; Status=&quot;HK&quot; ChangeOfPlane=&quot;false&quot; GuaranteedPaymentCarrier=&quot;No&quot; ProviderReservationInfoRef=&quot;8nfO9ysY1DKAvKUHAAAAAA==&quot; ProviderSegmentOrder=&quot;1&quot; OptionalServicesIndicator=&quot;true&quot; AvailabilitySource=&quot;S&quot; ParticipantLevel=&quot;Secure Sell&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;O and D cache or polled status used with different local status&quot;>
            &lt;air:FlightDetails Key=&quot;TLsU9ytY1DKANGdHAAAAAA==&quot; Origin=&quot;AMS&quot; Destination=&quot;BCN&quot; DepartureTime=&quot;2021-12-10T14:25:00.000+01:00&quot; ArrivalTime=&quot;2021-12-10T16:35:00.000+01:00&quot; FlightTime=&quot;130&quot; TravelTime=&quot;130&quot; Equipment=&quot;73H&quot; DestinationTerminal=&quot;1&quot; AutomatedCheckin=&quot;false&quot;/>
            &lt;common_v50_0:SellMessage>ARRIVES BCN TERMINAL 1&lt;/common_v50_0:SellMessage>
            &lt;common_v50_0:SellMessage>ADD ADVANCE PASSENGER INFORMATION SSRS DOCA/DOCO/DOCS&lt;/common_v50_0:SellMessage>
            &lt;common_v50_0:SellMessage>PERSONAL DATA WHICH IS PROVIDED TO US IN CONNECTION&lt;/common_v50_0:SellMessage>
            &lt;common_v50_0:SellMessage>WITH YOUR TRAVEL MAY BE PASSED TO GOVERNMENT AUTHORITIES&lt;/common_v50_0:SellMessage>
            &lt;common_v50_0:SellMessage>FOR BORDER CONTROL AND AVIATION SECURITY PURPOSES&lt;/common_v50_0:SellMessage>
         &lt;/air:AirSegment>
         &lt;air:SearchTraveler Key=&quot;8nfO9ysY1DKASJUHAAAAAA==&quot; Code=&quot;ADT&quot;>
            &lt;common_v50_0:Name First=&quot;Abraham Lincoln&quot; Last=&quot;Martin Luther King&quot;/>
         &lt;/air:SearchTraveler>
         &lt;air:HostReservation Carrier=&quot;KL&quot; CarrierLocatorCode=&quot;S9I4DN&quot; ProviderCode=&quot;1G&quot; ProviderLocatorCode=&quot;0CXQV4&quot;/>
      &lt;/air:SeatMapReq>
   &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
