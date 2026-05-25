//! Kithkin Zealot — `{1}{W}` 1/3 Kithkin Cleric. "When this creature enters,
//! you gain 1 life for each black and/or red permanent target opponent
//! controls."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kithkin Zealot");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::Opponent),
                    },
                ],
            }),
    )
}

fn on_etb(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(opp) = target else { return Vec::new(); };
    let black_count = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_colors(ColorSet::black())
            .controlled_by(ControllerConstraint::Opponent),
        *opp,
    );
    let red_count = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_colors(ColorSet::red())
            .controlled_by(ControllerConstraint::Opponent),
        *opp,
    );
    // "black and/or red" — count permanents that are black OR red; union
    // approximated by summing (may double-count multicolor permanents).
    // GAP: no set-union in ObjectFilter.
    let n = black_count + red_count;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife { player: trig.controller, amount: n }]
}
