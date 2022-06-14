<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Price_EDreams</name>
   <tag></tag>
   <elementGuidId>260a88c0-df3b-4973-b859-5e140e4a685e</elementGuidId>
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
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
   &lt;soap:Body>
      &lt;air:AirPriceReq AuthorizedBy=&quot;PANDAY&quot; TargetBranch=&quot;P7113930&quot; FareRuleType=&quot;none&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
         &lt;air:AirItinerary>
            &lt;air:AirSegment Key=&quot;8nfO9ysY1DKAKDFHAAAAAA==&quot; Group=&quot;0&quot; ProviderCode=&quot;1G&quot; Carrier=&quot;KL&quot; FlightNumber=&quot;1669&quot; Origin=&quot;AMS&quot; Destination=&quot;BCN&quot; DepartureTime=&quot;2021-12-28T10:05:00.000+01:00&quot; ArrivalTime=&quot;2021-12-28T12:31:00.000+01:00&quot; FlightTime=&quot;130&quot; Distance=&quot;771&quot; ETicketability=&quot;Yes&quot; Equipment=&quot;73H&quot; ChangeOfPlane=&quot;false&quot; ParticipantLevel=&quot;Secure Sell&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;Polled avail used&quot; OptionalServicesIndicator=&quot;false&quot; AvailabilitySource=&quot;S&quot; AvailabilityDisplayType=&quot;Fare Shop/Optimal Shop&quot;>&lt;/air:AirSegment>
         &lt;/air:AirItinerary>
         &lt;!--&lt;air:AirPricingModifiers ReturnFailedSegments=&quot;true&quot; InventoryRequestType=&quot;Seamless&quot;/>-->
         &lt;com:SearchPassenger Code=&quot;ADT&quot; BookingTravelerRef=&quot;1&quot;/>
         &lt;!--&lt;com:SearchPassenger Code=&quot;CHD&quot; BookingTravelerRef=&quot;2&quot;/>-->
         &lt;!--&lt;com:SearchPassenger Code=&quot;INF&quot; BookingTravelerRef=&quot;3&quot;/>-->
         &lt;!--&lt;com:SearchPassenger Code=&quot;ADT&quot; BookingTravelerRef=&quot;2&quot;/>-->
         &lt;!--com:SearchPassenger Code=&quot;CHD&quot; BookingTravelerRef=&quot;2&quot;/-->
         &lt;air:AirPricingCommand>
            &lt;!--&lt;air:AirSegmentPricingModifiers AirSegmentRef=&quot;xcdVLvsY1DKAST3rCAAAAA==&quot;>
               &lt;air:PermittedBookingCodes>
                  &lt;air:BookingCode Code=&quot;F&quot;/>
               &lt;/air:PermittedBookingCodes>
            &lt;/air:AirSegmentPricingModifiers>-->
         &lt;/air:AirPricingCommand>
      &lt;/air:AirPriceReq>
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
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/air_v51_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
