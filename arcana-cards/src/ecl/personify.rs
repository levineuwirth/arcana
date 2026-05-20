//! Personify — `{1}{W}` instant. "Exile target creature you control,
//! then return that card to the battlefield under its owner's
//! control. Create a 1/1 colorless Shapeshifter creature token with
//! changeling."
//!
//! The blink (exile-then-return-to-battlefield) has no single catalog
//! Effect (DelayedAction return targets hand, not battlefield); the
//! token half is emitted, the flicker is GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Personify");
    let _shape = reg.interner_mut().intern("Shapeshifter");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let _t: Vec<TargetRequirement> = vec![];
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target creature you control, then return that card to the battlefield under its owner's control. Create a 1/1 colorless Shapeshifter creature token with changeling.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: exile-then-return-to-battlefield flicker has no catalog Effect.
    let shape = reg.interner().lookup("Shapeshifter").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shape);
    let token = TokenDefinition {
        name: shape,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Changeling],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
