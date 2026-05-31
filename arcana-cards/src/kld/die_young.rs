//! Die Young — `{1}{B}` sorcery. "Choose target creature. You get
//! {E}{E} (two energy counters), then you may pay any amount of {E}.
//! The creature gets -1/-1 until end of turn for each {E} paid this
//! way."
//!
//! The energy GAIN is expressible (`Effect::GainEnergy`). The "pay any
//! amount of {E}" cost and the resulting dynamic -1/-1-per-{E}-paid
//! pump are NOT — spending energy as a cost is not modeled, so the
//! amount of energy paid cannot be computed at resolution. The pump's
//! magnitude is therefore a GAP rather than a wrong fixed literal.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Die Young");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature. You get {E}{E} (two energy counters), then you may pay any amount of {E}. The creature gets -1/-1 until end of turn for each {E} paid this way.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(_id) = target else { return Vec::new(); };
    // GAP: cannot pay energy as a cost (no energy-spend primitive), so
    // the number of {E} paid is uncomputable and the -1/-1-per-{E}-paid
    // pump cannot be sized. Emit only the energy gain faithfully.
    vec![Effect::GainEnergy { player: entry.controller, amount: 2 }]
}
