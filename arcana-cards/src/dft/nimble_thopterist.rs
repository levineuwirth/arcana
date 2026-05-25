//! Nimble Thopterist — `{3}{U}` 3/2 blue Vedalken Artificer.
//! "When this creature enters, create a 1/1 colorless Thopter
//! artifact creature token with flying." ETB trigger producing a
//! Thopter token.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nimble Thopterist");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let artificer = reg.interner_mut().intern("Artificer");
    // Pre-intern the token subtype so the trigger resolver can look
    // it up via the non-mut interner at resolve time.
    let _thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_thopter_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger resolution: create a 1/1 colorless Thopter artifact
/// creature token with flying under the controller's control.
fn create_thopter_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter = reg
        .interner()
        .lookup("Thopter")
        .expect("Thopter interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter);
    let token = TokenDefinition {
        name: thopter,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
