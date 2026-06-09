//! Soul Reap — `{1}{B}` sorcery. "Destroy target nongreen creature. Its
//! controller loses 3 life if you've cast another black spell this turn."
//! The rider is gated in the resolver on `script::spells_cast_this_turn`
//! with a black + controlled-by-you filter: at resolution Soul Reap's OWN
//! cast is already in the event log (and Soul Reap is black), so "another
//! black spell" means the count is >= 2.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Reap");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target nongreen creature. Its controller loses 3 life if you've cast another black spell this turn.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().without_colors(ColorSet::green()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::DestroyPermanent { target: *id }];
    // "if you've cast another black spell this turn": Soul Reap's own cast
    // is already in this turn's event log at resolution and is itself black,
    // so "another black spell" = total black casts by you >= 2.
    let black_casts = script::spells_cast_this_turn(
        state,
        &ObjectFilter::new()
            .with_colors(ColorSet::black())
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if black_casts >= 2 {
        if let Some(controller) = state.objects.get(*id).map(|o| o.controller) {
            effects.push(Effect::LoseLife { player: controller, amount: 3 });
        }
    }
    effects
}
