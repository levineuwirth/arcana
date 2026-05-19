//! Rite of Renewal — `{3}{G}` sorcery.
//! "Return up to two target permanent cards from your graveyard to your hand.
//!  Target player shuffles up to four target cards from their graveyard into their library.
//!  Exile Rite of Renewal."
//!
//! GAP: Multi-target graveyard-to-hand (up to two), opponent shuffles cards back into library,
//! and self-exile are not fully expressible together. Best effort: return first graveyard target
//! to hand only.

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
    let name = reg.interner_mut().intern("Rite of Renewal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to two target permanent cards from your graveyard to your hand. Target player shuffles up to four target cards from their graveyard into their library. Exile Rite of Renewal.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::permanent() },
                    count: TargetCount::UpTo(2),
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
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: multi-target graveyard-to-hand (up to two), opponent-choice shuffle-back, and self-exile
    let mut effects = Vec::new();
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    effects
}
