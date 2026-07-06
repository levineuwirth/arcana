//! Spoils of Evil — `{2}{B}` instant. "For each artifact or creature
//! card in target opponent's graveyard, add {C} and you gain 1 life."
//! N = artifact-or-creature count in target opponent's graveyard.
//! The script surface gives us `graveyard_matching` for per-player
//! filtered counts. Use that to size both AddMana and GainLife.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spoils of Evil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "For each artifact or creature card in target opponent's graveyard, add {C} and you gain 1 life.".into(),
                target_requirements: vec![TargetRequirement::target_opponent()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(opp) = target else { return Vec::new(); };
    let filter = ObjectFilter::new()
        .with_types_any(arcana_core::types::TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE));
    let n = script::graveyard_matching(state, &filter, *opp, entry.controller);
    vec![
        Effect::AddMana {
            player: entry.controller,
            mana: vec![ManaUnit::plain(ManaColor::Colorless, entry.source); n as usize],
        },
        Effect::GainLife { player: entry.controller, amount: n },
    ]
}
