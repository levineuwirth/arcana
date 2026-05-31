//! Invasion of Kylem // Valor's Reach Tag Team
//!
//! Front face — Battle — Siege ({2}{R}{W}), enters with 5 defense counters:
//! When this Siege enters, up to two target creatures each get +2/+0 and gain
//! vigilance and haste until end of turn.
//! Back face — Sorcery: Create two 3/2 red and white Warrior creature tokens with
//! "Whenever this token and at least one other creature token attack, put a
//! +1/+1 counter on this token."
//!
//! GAP: defeat -> exile-and-cast-the-back-face-transformed (CR 310.11) is not
//! auto-wired; the battle just goes to the graveyard when defeated. The back
//! face is authored for record but its sorcery is never cast by the engine, so
//! its token-creation spell ability is not reachable.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Kylem");
    let siege = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face — Sorcery (record only; defeat->cast not wired).
    let back_name = reg.interner_mut().intern("Valor's Reach Tag Team");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::white(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 5,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_pump,
                trigger_zones: Vec::new(),
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(2),
                        controller: None,
                    },
                ],
            }),
    )
}

fn etb_pump(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for target in &trig.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::Pump {
                target: *id,
                power: 2,
                toughness: 0,
                duration: Duration::EndOfTurn,
                keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Haste],
            });
        }
    }
    effects
}
