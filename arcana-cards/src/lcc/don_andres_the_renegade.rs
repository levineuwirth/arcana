//! Don Andres, the Renegade — `{1}{U}{B}{R}` 4/3 Legendary Vampire Pirate.
//!
//! Oracle:
//! * "Each creature you control but don't own gets +2/+2, has menace and
//!   deathtouch, and is a Pirate in addition to its other types." — a
//!   static continuous anthem keyed on ownership; not a triggered or
//!   activated ability and the "control but don't own" predicate is not
//!   expressible. GAP'd.
//! * "Whenever you cast a noncreature spell you don't own, create two
//!   tapped Treasure tokens." — a spell-cast trigger. The "you don't own"
//!   restriction on the cast spell is not expressible (no ownership
//!   filter), so this fires on any noncreature spell you cast; the
//!   "tapped" rider on the Treasures is a fidelity gap of
//!   `CreateCommodityToken`.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Don Andres, the Renegade");
    let vampire = reg.interner_mut().intern("Vampire");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static anthem "Each creature you control but don't own gets
    // +2/+2, has menace and deathtouch, and is a Pirate" — a continuous
    // ownership-keyed static, not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                // GAP: "you don't own" restriction on the cast spell — no
                // ownership filter; fires on any noncreature spell you cast.
                filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: make_treasures,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_treasures(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "create two tapped Treasure tokens" — the "tapped" rider is a
    // fidelity gap of CreateCommodityToken.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 2,
    }]
}
