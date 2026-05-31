//! The Bear Force Pilot — `{1}{G}` 2/2 legendary green Bear Pilot. "When
//! The Bear Force Pilot enters the battlefield, create a 0/0 colorless
//! artifact Vehicle token with flying, 'This creature gets +1/+1 for each
//! Bear you control,' and crew 2." We mint the 0/0 colorless artifact
//! Vehicle token with flying; the token's printed self-pump static ability
//! ("+1/+1 for each Bear you control") and its Crew 2 ability are not
//! expressible on a TokenDefinition (abilities: vec![]), so the token is a
//! best-effort 0/0 flier without those riders.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Bear Force Pilot");
    let bear = reg.interner_mut().intern("Bear");
    let pilot = reg.interner_mut().intern("Pilot");
    // Interned at register so the trigger's resolver can look it up at
    // resolve time via the non-mut interner.
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(pilot);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_vehicle_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_vehicle_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vehicle = reg.interner().lookup("Vehicle")
        .expect("Vehicle interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vehicle);
    // GAP: the token's printed self-pump static ("This creature gets +1/+1
    // for each Bear you control") and its Crew 2 ability are not expressible
    // on a TokenDefinition — abilities left empty.
    let token = TokenDefinition {
        name: vehicle,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
