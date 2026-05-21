//! Chandra's Revolution — `{3}{R}` sorcery. "Chandra's Revolution
//! deals 4 damage to target creature. Tap target land. That land
//! doesn't untap during its controller's next untap step." The
//! 'skip next untap step' rider on the tapped land isn't a catalog
//! primitive — we model damage + tap and GAP the don't-untap rider.

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
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra's Revolution");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Chandra's Revolution deals 4 damage to target creature. Tap target land. That land doesn't untap during its controller's next untap step.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new().with_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::Exactly(1),
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
    if let Some(TargetChoice::Object(c)) = entry.targets.targets.first() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*c),
            amount: 4,
        });
    }
    if let Some(TargetChoice::Object(l)) = entry.targets.targets.get(1) {
        effects.push(Effect::Tap { target: *l });
        // GAP: 'doesn't untap during its controller's next untap step'
        // is not a catalog primitive.
    }
    effects
}
