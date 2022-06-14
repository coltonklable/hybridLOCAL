<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AirPrice Request</name>
   <tag></tag>
   <elementGuidId>7af0b2ab-5c97-49df-a0f0-a648048607fc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
	&lt;soap:Body>
		&lt;air:AirPriceReq AuthorizedBy=&quot;PANDAY&quot; TargetBranch=&quot;P7113930&quot; FareRuleType=&quot;none&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
			&lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
			&lt;air:AirItinerary>
				  &lt;air:AirSegment Key=&quot;REfX7wtY1DKAbLrDAAAAAA==&quot; Group=&quot;0&quot; ProviderCode=&quot;1G&quot; Carrier=&quot;${Carrier}&quot; FlightNumber=&quot;${FlightNumber}&quot; Origin=&quot;${Origin}&quot; Destination=&quot;${Destination}&quot; DepartureTime=&quot;${DepartureTime}&quot; ArrivalTime=&quot;${ArrivalTime}&quot; Distance=&quot;${Distance}&quot; ETicketability=&quot;Yes&quot; Equipment=&quot;${Equipment}&quot; ChangeOfPlane=&quot;false&quot; ParticipantLevel=&quot;Secure Sell&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;Cached status used. Polled avail exists&quot; OptionalServicesIndicator=&quot;false&quot; AvailabilitySource=&quot;Q&quot; AvailabilityDisplayType=&quot;Fare Shop/Optimal Shop&quot;>&lt;/air:AirSegment>
               
			&lt;/air:AirItinerary>
			&lt;com:SearchPassenger Code=&quot;ADT&quot; BookingTravelerRef=&quot;1&quot;/>
			&lt;air:AirPricingCommand>
			&lt;/air:AirPricingCommand>
		&lt;/air:AirPriceReq>
	&lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Carrier</defaultValue>
      <description></description>
      <id>c6ae792b-2346-47ef-9d58-e5d064e6f85f</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.FlightNumber</defaultValue>
      <description></description>
      <id>32c3f922-2ed5-4e8d-86fa-bcad483b4709</id>
      <masked>false</masked>
      <name>FlightNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Origin</defaultValue>
      <description></description>
      <id>f90bbf83-8254-420d-9403-00ad75b3281c</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Destination</defaultValue>
      <description></description>
      <id>dda3bb20-6d4e-4977-9a9c-0847252324fb</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DepartureTime</defaultValue>
      <description></description>
      <id>5aea3cb6-b7c4-4bec-8389-585736736d32</id>
      <masked>false</masked>
      <name>DepartureTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DepartureTime1</defaultValue>
      <description></description>
      <id>946476a8-924b-44f0-80a5-fc11c190bc98</id>
      <masked>false</masked>
      <name>DepartureTime1</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ArrivalTime</defaultValue>
      <description></description>
      <id>a24e48ca-d483-40bb-be94-8a607ae56481</id>
      <masked>false</masked>
      <name>ArrivalTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ArrivalTime1</defaultValue>
      <description></description>
      <id>08925b56-4b83-43ed-bd46-64849d3b6855</id>
      <masked>false</masked>
      <name>ArrivalTime1</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Distance</defaultValue>
      <description></description>
      <id>3c01331f-3a91-4766-bd7d-bcd1b0ce5833</id>
      <masked>false</masked>
      <name>Distance</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Equipment</defaultValue>
      <description></description>
      <id>8ced099b-95c4-482d-987b-616370192958</id>
      <masked>false</masked>
      <name>Equipment</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
