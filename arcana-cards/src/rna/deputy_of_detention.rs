//! Deputy of Detention — `{1}{W}{U}` 1/3 blue/white Vedalken Wizard. "When this creature
//! enters, exile target nonland permanent an opponent controls and all other nonland
//! permanents that player controls with the same name as that permanent until this
//! creature leaves the battlefield."
//! Wired via Effect::ExileUntilSourceLeaves — the target plus all other nonland
//! permanents that player controls with the same name are exiled and linked; the
//! engine returns them when this creature leaves the battlefield.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement, TargetChoice};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deputy of Detention");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
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
                effect: etb_exile_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .without_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_exile_nonland(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // The target plus all other nonland permanents that player controls with
    // the same name; each is linked via ExileUntilSourceLeaves so the engine
    // returns them when this creature leaves the battlefield.
    let mut ids = vec![*id];
    if let Some(t) = state.objects.get(*id) {
        let same_name = t.characteristics.name;
        let that_player = t.controller;
        ids.extend(
            state
                .objects_in_zone(Zone::Battlefield)
                .filter(|o| {
                    o.id != *id
                        && o.controller == that_player
                        && o.characteristics.name == same_name
                        && !o.is_land()
                })
                .map(|o| o.id),
        );
    }
    ids.into_iter()
        .map(|exiled| Effect::ExileUntilSourceLeaves {
            source: trig.source,
            target: exiled,
        })
        .collect()
}
