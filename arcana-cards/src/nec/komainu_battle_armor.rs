//! Komainu Battle Armor — `{2}{R}` 2/2 Artifact Creature — Equipment Dog.
//! Menace.
//! "Equipped creature gets +2/+2 and has menace." (equip static — GAP)
//! "Whenever this creature or equipped creature deals combat damage to a
//! player, goad each creature that player controls."
//! "Reconfigure {4}." (GAP — Equipment reconfigure not expressible)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Komainu Battle Armor");
    let equipment = reg.interner_mut().intern("Equipment");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(dog);
    // GAP (static): "Equipped creature gets +2/+2 and has menace" — an equip
    // static; no equipped-creature continuous-effect slot in this shape.
    // GAP (keyword): "Reconfigure {4}" — not in the usable KeywordAbility set
    // and not expressible as an attach/unattach activated ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: goad_their_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn goad_their_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "goad each creature that player controls" — the damaged player's creatures.
    let Some(them) = trig.damaged_player() else { return Vec::new(); };
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        them,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Goad {
            target: NULL_OBJECT_ID,
            goader: trig.controller,
            duration: Duration::EndOfTurn,
        }),
    }]
}
