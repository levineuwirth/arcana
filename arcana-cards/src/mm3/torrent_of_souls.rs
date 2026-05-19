//! Torrent of Souls — `{4}{B/R}` sorcery. "Return up to one target creature
//! card from your graveyard to the battlefield if {B} was spent to cast this
//! spell. Creatures target player controls get +2/+0 and gain haste until end
//! of turn if {R} was spent to cast this spell. (Do both if {B}{R} was spent.)"
//!
//! # GAP: mana-spent conditional ({B} / {R} checks) not in Effect catalog
//! Best-effort: emit both effects unconditionally (they are always both possible
//! when the hybrid {B/R} is paid with either color).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement, TargetChoice};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Torrent of Souls");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to one target creature card from your graveyard to the battlefield if {B} was spent to cast this spell. Creatures target player controls get +2/+0 and gain haste until end of turn if {R} was spent to cast this spell. (Do both if {B}{R} was spent.)".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement::target_player(),
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: mana-spent conditional ({B}/{R} checks) not in Effect catalog
    // Emitting both effects unconditionally as best-effort.
    let mut effects = Vec::new();
    // First target: creature card from graveyard
    if let Some(t) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
        }
    }
    // Second target: player whose creatures get pumped
    if let Some(t) = entry.targets.targets.get(1) {
        if let TargetChoice::Player(p) = t {
            let creatures = script::ids_matching(
                state,
                &ObjectFilter::creature().controlled_by(arcana_core::targets::ControllerConstraint::You),
                *p,
            );
            for id in creatures {
                effects.push(Effect::Pump {
                    target: id,
                    power: 2,
                    toughness: 0,
                    duration: Duration::EndOfTurn,
                    keywords: vec![KeywordAbility::Haste],
                });
            }
        }
    }
    effects
}
