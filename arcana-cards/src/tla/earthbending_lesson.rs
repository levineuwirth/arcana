//! Earthbending Lesson — `{3}{G}` sorcery (Lesson). "Earthbend 4.
//! (Target land you control becomes a 0/0 creature with haste that's
//! still a land. Put four +1/+1 counters on it. When it dies or is
//! exiled, return it to the battlefield tapped.)"
//!
//! Best-effort: the spell targets a land you control, animates it into
//! a 0/0 creature (still a land) with haste, and puts four +1/+1
//! counters on it. The dies-or-exiled-return rider cannot be attached
//! to the target permanent from a spell resolver, so it is GAP-ed.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Earthbending Lesson");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Earthbend 4. (Target land you control becomes a 0/0 creature with haste that's still a land. Put four +1/+1 counters on it. When it dies or is exiled, return it to the battlefield tapped.)".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "when it dies or is exiled, return it to the battlefield
    // tapped" — no primitive to attach a dies/exile-return delayed
    // trigger to a targeted permanent from a spell resolver.
    vec![
        Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: *id,
            power: 0,
            toughness: 0,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 4,
        },
    ]
}
