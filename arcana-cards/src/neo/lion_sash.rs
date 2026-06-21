//! Lion Sash — `{1}{W}` 1/1 Artifact Creature — Equipment Cat.
//!
//! Oracle:
//! * "{W}: Exile target card from a graveyard. If it was a permanent
//!   card, put a +1/+1 counter on this permanent." — an activated
//!   graveyard-hate ability. The exile is modeled; the conditional
//!   "if it was a permanent card, put a +1/+1 counter" is a
//!   resolution-time type check on the exiled card that no primitive
//!   expresses, so the counter half is GAP'd.
//! * "Equipped creature gets +1/+1 for each +1/+1 counter on this
//!   Equipment." — a continuous static equip buff, GAP'd.
//! * "Reconfigure {2}." — the Reconfigure keyword is not an engine
//!   KeywordAbility variant, GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lion Sash");
    let equipment = reg.interner_mut().intern("Equipment");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "Equipped creature gets +1/+1 for each +1/+1 counter
    // on this Equipment" — a continuous equip buff, not expressible.
    // GAP: "Reconfigure {2}" — no Reconfigure KeywordAbility variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}: Exile target card from a graveyard. If it was a permanent card, put a +1/+1 counter on this permanent.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::default(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_from_graveyard,
            }),
    )
}

fn exile_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP fidelity: "if it was a permanent card, put a +1/+1 counter on
    // this permanent" — no resolution-time type check on the exiled
    // card is expressible, so the conditional counter is omitted.
    vec![Effect::ExileFromGraveyard { target: *id }]
}
