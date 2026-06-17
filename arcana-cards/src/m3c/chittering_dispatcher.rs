//! Chittering Dispatcher — `{2}{G}` 2/3 Eldrazi Drone.
//! Devoid (colorless) and Myriad. "When this creature leaves the
//! battlefield, create a 0/1 colorless Eldrazi Spawn creature token
//! with 'Sacrifice this token: Add {C}.'"
//!
//! Devoid and Myriad are not in the supported KeywordAbility surface,
//! so `keywords: vec![]` (the color is set to colorless to honor Devoid).
//! The leaves-the-battlefield token is wired; the token's printed
//! sacrifice-for-mana ability is not authored on the TokenDefinition.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chittering Dispatcher");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    // Pre-intern the token's primary subtype so the resolver can look it up.
    let _spawn = reg.interner_mut().intern("Eldrazi Spawn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        // Devoid: this card has no color.
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Devoid and Myriad are not in the supported keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: leaves_make_spawn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn leaves_make_spawn(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spawn = reg.interner().lookup("Eldrazi Spawn").unwrap_or_default();
    // GAP: token's "Sacrifice this token: Add {C}" ability not authored.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spawn,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes: {
                let mut s = SubtypeSet::default();
                s.0.insert(spawn);
                s
            },
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
