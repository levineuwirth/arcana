//! Bond of Passion — `{4}{R}{R}` sorcery, "Gain control of target
//! creature until end of turn. Untap that creature. It gains haste
//! until end of turn. Bond of Passion deals 2 damage to any other
//! target."
//!
//! GAP: "gain control of target creature until end of turn" has no
//! corresponding Effect. The untap, haste grant, and the 2-damage
//! clause are modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bond of Passion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Gain control of target creature until end of turn. Untap that creature. It gains haste until end of turn. Bond of Passion deals 2 damage to any other target.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::creature()),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::any_target(),
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
    let mut iter = entry.targets.targets.iter();
    let Some(TargetChoice::Object(cid)) = iter.next() else { return Vec::new(); };
    let mut effects = vec![
        // GAP: gain-control-until-end-of-turn not expressible.
        Effect::Untap { target: *cid },
        Effect::GrantKeyword {
            target: *cid,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ];
    if let Some(t) = iter.next() {
        let dt = match t {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
        };
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: dt,
            amount: 2,
        });
    }
    effects
}
