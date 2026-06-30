//! Kolaghan's Command — `{1}{B}{R}` modal instant. "Choose two — Kolaghan's
//! Command deals 2 damage to any target; or return target creature card from a
//! graveyard to its owner's hand; or destroy target artifact; or target player
//! discards a card." (The printed mode 1 reads "your graveyard"; the Card-in-
//! graveyard target matches any graveyard — minor over-permissiveness.)
//!
//! Choose-two modal: the chosen clauses' targets arrive concatenated in card
//! order, and every mode here has exactly one target, so `resolve` walks the
//! chosen mode indices (already sorted into card order) advancing a one-per-mode
//! cursor into `entry.targets`.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardRegistry, ModalSpec, ModeClause, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kolaghan's Command");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
        text: "Choose two — deal 2 damage to any target; return target creature card from a graveyard to its owner's hand; destroy target artifact; or target player discards a card.".into(),
        target_requirements: vec![],
        modal: Some(ModalSpec {
            min_modes: 2,
            max_modes: 2,
            clauses: vec![
                ModeClause {
                    text: "Kolaghan's Command deals 2 damage to any target.".into(),
                    target_requirements: vec![TargetRequirement::any_target()],
                },
                ModeClause {
                    text: "Return target creature card from a graveyard to its owner's hand.".into(),
                    target_requirements: vec![TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    }],
                },
                ModeClause {
                    text: "Destroy target artifact.".into(),
                    target_requirements: vec![TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter {
                            types: Some(TypeLine::ARTIFACT.into()),
                            ..Default::default()
                        }),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    }],
                },
                ModeClause {
                    text: "Target player discards a card.".into(),
                    target_requirements: vec![TargetRequirement::target_player()],
                },
            ],
        }),
        effect: resolve,
    }))
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(choice) = entry.modes.first() else { return Vec::new(); };
    let mut out = Vec::new();
    let mut cursor = 0usize;
    for &mode in &choice.mode_indices {
        let target = entry.targets.targets.get(cursor);
        cursor += 1;
        match (mode, target) {
            (0, Some(t)) => {
                let dt = match t {
                    TargetChoice::Object(id) => DamageTarget::Object(*id),
                    TargetChoice::Player(p) => DamageTarget::Player(*p),
                    TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
                        DamageTarget::Object(*id)
                    }
                    TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
                        DamageTarget::Player(*p)
                    }
                };
                out.push(Effect::DealDamage { source: entry.source, target: dt, amount: 2 });
            }
            (1, Some(TargetChoice::Object(id))) => {
                out.push(Effect::ReturnFromGraveyardToHand { target: *id });
            }
            (2, Some(TargetChoice::Object(id))) => {
                out.push(Effect::DestroyPermanent { target: *id });
            }
            (3, Some(TargetChoice::Player(p))) => {
                out.push(Effect::Discard {
                    player: *p,
                    count: 1,
                    choice: DiscardChoice::ControllerChooses,
                });
            }
            _ => {}
        }
    }
    out
}
