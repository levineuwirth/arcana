//! Giant's Ire — `{3}{R}` Kindred Sorcery — Giant. "Giant's Ire deals
//! 4 damage to target player or planeswalker. If you control a Giant,
//! draw a card." We can target player-or-planeswalker via
//! AnyTarget (broader); 'Kindred' type isn't a TypeLine const, treat
//! as plain SORCERY. The Giant-condition draw uses subtype_filter and
//! count_matching > 0 → draw a card via ForEach over a 1-element vec
//! (since Conditional needs a `condition` we don't have a primitive
//! for); best-effort: count Giants you control and emit Draw N
//! capped at 1.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant's Ire");
    let _giant = reg.interner_mut().intern("Giant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    // GAP: 'Kindred' type designation (treated as plain Sorcery).
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Giant's Ire deals 4 damage to target player or planeswalker. If you control a Giant, draw a card.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    let mut effects = vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 4,
    }];
    let giants = script::count_matching(
        state,
        &script::subtype_filter(reg, "Giant").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if giants > 0 {
        effects.push(Effect::DrawCards { player: entry.controller, count: 1 });
    }
    let _ = ObjectFilter::creature();
    effects
}
