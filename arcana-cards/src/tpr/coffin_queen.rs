//! Coffin Queen — `{2}{B}` 1/1 Zombie Wizard.
//! You may choose not to untap this creature during your untap step. (static — GAP)
//! {2}{B}, {T}: Put target creature card from a graveyard onto the
//! battlefield under your control. (The "when this becomes untapped or you
//! lose control, exile that creature" delayed rider is GAP'd.)

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
    let name = reg.interner_mut().intern("Coffin Queen");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static — "You may choose not to untap this creature during your
    // untap step." No primitive for an optional skip-untap restriction.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{B}, {T}: Put target creature card from a graveyard onto the battlefield under your control."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: reanimate_under_your_control,
        }),
    )
}

fn reanimate_under_your_control(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // The reanimated card enters; "under your control" + the becomes-untapped/
    // lose-control exile rider is GAP'd (no delayed exile keyed to untap/control
    // change on the reanimated object).
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::ChangeControl {
            target: *id,
            new_controller: ctx.controller,
        },
    ]
}
