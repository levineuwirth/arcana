//! Repair and Recharge — `{3}{W}{W}` sorcery. "Return target artifact,
//! enchantment, or planeswalker card from your graveyard to the
//! battlefield. Create a tapped Powerstone token."
//!
//! GAP: token's "{T}: Add {C}. Can't be spent to cast nonartifact" is
//! the activated mana ability — not expressible in TokenDefinition's
//! `abilities: vec![]`. The Powerstone token shell is emitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Repair and Recharge");
    let _powerstone = reg.interner_mut().intern("Powerstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target artifact, enchantment, or planeswalker card from your graveyard to the battlefield. Create a tapped Powerstone token.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .with_types_any(TypeLine::ARTIFACT.into())
                        .with_types_any(TypeLine::ENCHANTMENT.into())
                        .with_types_any(TypeLine::PLANESWALKER.into()),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let powerstone = reg.interner().lookup("Powerstone").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(powerstone);
    let token = TokenDefinition {
        name: powerstone,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "tapped" on entry and the Powerstone mana ability.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::CreateToken {
            controller: entry.controller,
            token,
        },
    ]
}
