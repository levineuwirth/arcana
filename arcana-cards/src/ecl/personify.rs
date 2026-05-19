//! Personify — `{1}{W}` instant. "Exile target creature you control, then return it to the
//! battlefield under its owner's control. Create a 1/1 colorless Shapeshifter creature token
//! with changeling."
//!
//! # GAP: ExilePermanent + ReturnFromGraveyardToBattlefield doesn't cleanly model
//! "exile then return the same permanent to the battlefield" (zone-identity preserved).
//! Partial: ExilePermanent + CreateToken Shapeshifter with Changeling keyword.
//! ReturnFromGraveyardToBattlefield is omitted since the card is exiled (not in graveyard).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Personify");
    let _shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature you control, then return it to the battlefield under its owner's control. Create a 1/1 colorless Shapeshifter creature token with changeling.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let shapeshifter = reg.interner().lookup("Shapeshifter")
        .expect("Shapeshifter interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    let token = TokenDefinition {
        name: shapeshifter,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Changeling],
        abilities: vec![],
    };
    // GAP: "exile then return same permanent to battlefield" (not via graveyard) not in engine catalog
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
