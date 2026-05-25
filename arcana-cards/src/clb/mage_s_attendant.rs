//! Mage's Attendant — `{2}{W}` 3/2 white Cat Rogue. "When this
//! creature enters, create a 1/1 blue Wizard creature token with
//! `{1}, Sacrifice this token: Counter target noncreature spell
//! unless its controller pays {1}.`"
//!
//! The ETB trigger is the canonical `SelfEntersBattlefield` →
//! `CreateToken` shape. The Wizard token's activated counter ability
//! is NOT expressible — `TokenDefinition.abilities` is `vec![]` here
//! and that activated ability is left as a GAP for the verify
//! pipeline to flag.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mage's Attendant");
    let cat = reg.interner_mut().intern("Cat");
    let rogue = reg.interner_mut().intern("Rogue");
    // Pre-intern the token's subtype so the trigger resolver can
    // look it up via the non-mut interner at resolve time.
    let _wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
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
                effect: etb_create_wizard_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger resolution: create a 1/1 blue Wizard creature token.
///
/// GAP: the token's printed activated ability "{1}, Sacrifice this
/// token: Counter target noncreature spell unless its controller
/// pays {1}." is not expressible via `TokenDefinition.abilities` in
/// the demonstrated API; the token is emitted as a vanilla 1/1 blue
/// Wizard.
fn etb_create_wizard_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wizard = reg
        .interner()
        .lookup("Wizard")
        .expect("Wizard interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    let token = TokenDefinition {
        name: wizard,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
