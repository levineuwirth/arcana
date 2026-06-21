//! Virtus the Veiled — `{2}{B}` 1/1 Legendary Azra Assassin with Deathtouch.
//!
//! * Deathtouch — base keyword.
//! * "Partner with Gorm the Great" — Partner / Partner-with is not a usable
//!   `KeywordAbility` variant and the tutor-on-ETB partner mechanic is
//!   unmodeled, so it is GAP'd.
//! * "Whenever Virtus deals combat damage to a player, that player loses half
//!   their life, rounded up." — a `DamageDealt` (combat, to a player) trigger;
//!   the damaged player loses ceil(life / 2).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Virtus the Veiled");
    let azra = reg.interner_mut().intern("Azra");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(azra);
    subtypes.0.insert(assassin);

    // GAP: "Partner with Gorm the Great" — Partner/Partner-with is not modeled.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_halve,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_halve(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    let life = script::life(state, p).max(0) as u32;
    let amount = (life + 1) / 2; // half their life, rounded up
    vec![Effect::LoseLife { player: p, amount }]
}
