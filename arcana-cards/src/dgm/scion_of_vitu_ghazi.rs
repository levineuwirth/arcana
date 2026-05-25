//! Scion of Vitu-Ghazi — `{3}{W}{W}` 4/4 white creature. "When this
//! creature enters, if you cast it from your hand, create a 1/1 white
//! Bird creature token with flying, then populate."
//!
//! Keywords: Populate is not in the supported keyword list — emitting
//! keywords: vec![].
//!
//! GAP: intervening_if — "if you cast it from your hand" not expressible;
//! using None.
//! GAP: effect — populate (copy a creature token you control) not
//! expressible; creating the Bird token only.

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
    let name = reg.interner_mut().intern("Scion of Vitu-Ghazi");
    let elemental = reg.interner_mut().intern("Elemental");
    let _bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening_if — "if you cast it from your hand" not expressible
                intervening_if: None,
                effect: etb_create_bird_then_populate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_create_bird_then_populate(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — populate not expressible; creating Bird token only
    let bird = reg.interner().lookup("Bird")
        .expect("Bird interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(bird);
    let token = TokenDefinition {
        name: bird,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
