//! Mysidian Elder — `{2}{R}` 1/3 red Human Wizard.
//! "When this creature enters, create a 0/1 black Wizard creature token with
//! 'Whenever you cast a noncreature spell, this token deals 1 damage to each opponent.'"
//!
//! GAP: token's triggered ability ("Whenever you cast a noncreature spell, this token
//! deals 1 damage to each opponent") cannot be represented — the engine's TokenDefinition
//! does not support triggered abilities on tokens authored inline.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Mysidian Elder");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    // Pre-intern so the trigger resolver can look it up at resolve time.
    let _wizard_token = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_wizard_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_wizard_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wizard = reg.interner().lookup("Wizard")
        .expect("Wizard interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    let token = TokenDefinition {
        name: wizard,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: token's triggered ability ("Whenever you cast a noncreature spell,
        // this token deals 1 damage to each opponent") is not expressible via
        // TokenDefinition.abilities — inline triggered abilities on tokens are
        // not supported.
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
