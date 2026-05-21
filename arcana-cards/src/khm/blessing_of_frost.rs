//! Blessing of Frost — `{3}{G}` Snow Sorcery. "Distribute X +1/+1 counters
//! among any number of creatures you control, where X is the amount of {S}
//! spent to cast this spell. Then draw a card for each creature you control
//! with power 4 or greater." The {S}-spent amount X is not exposed by any
//! script helper, so the counter distribution is gapped; the conditional
//! draw is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blessing of Frost");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Distribute X +1/+1 counters among any number of creatures you control, where X is the amount of {S} spent to cast this spell. Then draw a card for each creature you control with power 4 or greater.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: X = the amount of {S} (snow mana) spent to cast this spell is not
    // exposed by any script helper, so the +1/+1 counter distribution is omitted.
    let draws = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_min_power(4),
        entry.controller,
    );
    if draws == 0 {
        Vec::new()
    } else {
        vec![Effect::DrawCards { player: entry.controller, count: draws }]
    }
}
