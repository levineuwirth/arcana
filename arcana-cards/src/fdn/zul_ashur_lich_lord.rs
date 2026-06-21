//! Zul Ashur, Lich Lord — `{1}{B}` 2/2 Legendary Zombie Warlock.
//! Ward—Pay 2 life. (non-mana ward — GAP'd; Ward only models a mana cost.)
//! {T}: You may cast target Zombie creature card from your graveyard this
//! turn.
//!
//! Ward—Pay 2 life has a non-mana cost, which the Ward keyword (mana cost
//! only) cannot express, so the keyword line is empty and it is GAP'd.
//! The activated ability targets a Zombie creature card in a graveyard
//! and casts it via Effect::CastFromGraveyard. The "this turn" permission
//! window is approximated by casting it on resolution (a partial: it is
//! cast immediately rather than gaining a delayed cast permission).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zul Ashur, Lich Lord");
    let zombie = reg.interner_mut().intern("Zombie");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(warlock);

    let zombie_creature =
        script::subtype_filter(reg, "Zombie").with_types(TypeLine::CREATURE.into());

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP (keyword): "Ward—Pay 2 life." — Ward only models a mana cost; a
    // pay-life ward is not expressible.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: You may cast target Zombie creature card from your graveyard this turn."
                .into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: zombie_creature,
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: cast_zombie,
        }),
    )
}

fn cast_zombie(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::CastFromGraveyard {
        player: ctx.controller,
        target: *id,
    }]
}
