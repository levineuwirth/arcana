//! Tlincalli Hunter // Retrieve Prey — `{5}{G}{G}` / `{1}{G}` Adventure
//!
//! Creature: `{5}{G}{G}` Creature — Scorpion Scout (7/7)
//!   Trample.
//!   Once each turn, you may pay {0} rather than pay the mana cost for a
//!   creature spell you cast from exile. (GAP: cost-substitution deferred.)
//!
//! Adventure: `{1}{G}` Sorcery — Retrieve Prey
//!   Exile target creature card from your graveyard. Until end of next turn,
//!   you may cast that card. (GAP: exile-with-cast-permission effect not in
//!   Effect API; emitting Vec::new().)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tlincalli Hunter");
    let scorpion_sub = reg.interner_mut().intern("Scorpion");
    let scout_sub = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scorpion_sub);
    subtypes.0.insert(scout_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Trample],
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Retrieve Prey");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Exile target creature card from your graveyard. Until the end of your next turn, you may cast that card.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Card {
                zone: Zone::Graveyard(0),
                filter: arcana_core::targets::ObjectFilter::creature(),
            },
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile-with-cast-permission-until-next-turn not in Effect API
    Vec::new()
}
