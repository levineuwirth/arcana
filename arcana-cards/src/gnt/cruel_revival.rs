//! Cruel Revival — `{4}{B}` instant, "Destroy target non-Zombie creature. It
//! can't be regenerated. Return up to one target Zombie card from your graveyard
//! to your hand."
//!
//! # GAP: "non-Zombie" subtype exclusion filter not in ObjectFilter
//! # GAP: "can't be regenerated" effect not in catalog
//! # GAP: UpTo filter for graveyard target (up to one Zombie card) partially
//!        represented as best-effort single target

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Revival");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target non-Zombie creature. It can't be regenerated. Return up to one target Zombie card from your graveyard to your hand.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(first) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = first {
            // GAP: "non-Zombie" filter and "can't be regenerated" not expressible
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    if let Some(second) = entry.targets.targets.get(1) {
        if let TargetChoice::Object(id) = second {
            // GAP: Zombie subtype filter on graveyard target not expressible
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    effects
}
