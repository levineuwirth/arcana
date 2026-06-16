//! General's Enforcer — `{W}{B}` 2/3 Human Soldier.
//!
//! Oracle text:
//! * "Legendary Humans you control have indestructible." — a pure
//!   static continuous ability; the MultiAbilityCreature shape can't
//!   express it. GAP'd below.
//! * "{2}{W}{B}: Exile target card from a graveyard. If it was a
//!   creature card, create a 1/1 white Human Soldier creature token."
//!   — the exile half is wired as an activated ability via
//!   `Effect::ExileFromGraveyard`; the "if it was a creature card →
//!   token" rider is a property of the (now exiled) card and isn't
//!   cleanly expressible, so that conditional is GAP'd.

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
    let name = reg.interner_mut().intern("General's Enforcer");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static — "Legendary Humans you control have indestructible."
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{W}{B}: Exile target card from a graveyard. If it was a creature card, create a 1/1 white Human Soldier creature token.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{W}{B}").expect("valid cost"),
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
            effect: exile_target_card,
        }),
    )
}

fn exile_target_card(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "If it was a creature card, create a 1/1 white Human Soldier
    // token." — the token rider is conditional on a property of the
    // exiled card and isn't cleanly expressible here.
    vec![Effect::ExileFromGraveyard { target: *id }]
}
