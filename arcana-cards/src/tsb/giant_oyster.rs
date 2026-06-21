//! Giant Oyster — `{2}{U}{U}` 0/3 blue Oyster.
//!
//! * "You may choose not to untap this creature during your untap step."
//!   A static untap-restriction choice; not expressible with the
//!   demonstrated API, GAP'd.
//! * "{T}: For as long as this creature remains tapped, target tapped
//!   creature doesn't untap during its controller's untap step, and at
//!   the beginning of each of your draw steps, put a -1/-1 counter on
//!   that creature. When this creature leaves the battlefield or becomes
//!   untapped, remove all -1/-1 counters from the creature." The {T}
//!   cost and the "target tapped creature" requirement are recorded, but
//!   the linked behavior (a "for as long as tapped" don't-untap lock, a
//!   recurring per-draw-step -1/-1 counter, and the leaves/untaps
//!   cleanup) is not expressible — the effect body is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Oyster");
    let oyster = reg.interner_mut().intern("Oyster");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(oyster);

    // GAP: static "you may choose not to untap this creature during your
    // untap step" — no untap-restriction choice primitive available.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: For as long as this creature remains tapped, target tapped \
                   creature doesn't untap during its controller's untap step, and at \
                   the beginning of each of your draw steps, put a -1/-1 counter on \
                   that creature."
                .into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().tapped_only()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: oyster_lock,
        }),
    )
}

fn oyster_lock(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "for as long as this remains tapped" don't-untap lock, the
    // recurring per-draw-step -1/-1 counter, and the leaves/untaps
    // cleanup form one linked continuous ability with no demonstrated
    // primitive — emitting a fixed one-shot effect would misrepresent it.
    Vec::new()
}
