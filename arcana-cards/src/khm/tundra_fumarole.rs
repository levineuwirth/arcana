//! Tundra Fumarole — `{1}{R}{R}` snow sorcery, "Tundra Fumarole deals
//! 4 damage to target creature or planeswalker. Add {C} for each {S}
//! spent to cast this spell. Until end of turn, you don't lose this
//! mana as steps and phases end."
//!
//! The 4 damage to a creature-or-planeswalker target is expressible.
//! The "Add {C} for each {S} spent to cast this spell" rider depends on
//! the snow-mana provenance of mana paid for the spell, which the card
//! script helpers do not expose, and the "doesn't empty as steps/phases
//! end" mana-retention clause has no effect primitive — both are GAPs.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tundra Fumarole");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Tundra Fumarole deals 4 damage to target creature or planeswalker. \
                       Add {C} for each {S} spent to cast this spell. Until end of turn, \
                       you don't lose this mana as steps and phases end."
                    .into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::CREATURE | TypeLine::PLANESWALKER,
                        )),
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "Add {C} for each {S} spent to cast this spell" and the
    // "doesn't empty as steps/phases end" retention clause are not
    // expressible — no access to snow-mana provenance of the cast cost.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}
