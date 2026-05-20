//! Funeral Pyre — `{W}` instant. "Exile target card from a
//! graveyard. Its owner creates a 1/1 white Spirit creature token
//! with flying."
//!
//! The exiled card's owner is not addressable as a player for the
//! token payout; we exile the targeted graveyard card and create the
//! Spirit token for the caster's controller as a best-effort (owner
//! attribution is gapped).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Funeral Pyre");
    let _sp = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target card from a graveyard. Its owner creates a 1/1 white Spirit creature token with flying.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::default(),
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
    let sp = reg.interner().lookup("Spirit").expect("Spirit interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sp);
    let token = TokenDefinition {
        name: sp,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    // GAP: the exiled card's owner is not addressable; token payout
    // goes to the caster's controller as a best-effort.
    vec![
        Effect::ExileFromGraveyard { target: *id },
        Effect::CreateToken {
            controller: entry.controller,
            token,
        },
    ]
}
