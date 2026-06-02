//! Enduring Angel // Angelic Enforcer — `{2}{W}{W}{W}` Creature — Angel 3/3 (transform TDFC).
//!
//! Front face (Enduring Angel):
//!   Flying, double strike.
//!   You have hexproof.
//!   If your life total would be reduced to 0 or less, instead transform this creature and your
//!   life total becomes 3. Then if this creature didn't transform this way, you lose the game.
//!
//! Back face (Angelic Enforcer):
//!   Flying.
//!   You have hexproof.
//!   Angelic Enforcer's power and toughness are each equal to your life total.
//!   Whenever this creature attacks, double your life total.
//!
//! GAP: "You have hexproof" (player-granted hexproof) — no static-ability mechanism to grant a
//!      keyword to a player.
//! GAP: front-face life-total replacement ("if your life total would be reduced to 0 or less,
//!      instead transform … and your life total becomes 3; else you lose the game") — no
//!      replacement effect for life-total reduction; not expressible.
//! GAP: back-face "power and toughness are each equal to your life total" — no characteristic-
//!      defining static for P/T from life total; the back face uses a placeholder P/T.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enduring Angel");
    let angel_sub = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // Back face: Angelic Enforcer — Angel with Flying.
    let back_name = reg.interner_mut().intern("Angelic Enforcer");
    let back_angel_sub = reg.interner_mut().intern("Angel");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_angel_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // GAP: actual P/T are "each equal to your life total"; placeholder used.
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back-face only: "Whenever this creature attacks, double your life total."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_double_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
        // GAP: "You have hexproof" on both faces — player-granted keyword not expressible.
        // GAP: front-face life-reduction replacement → transform / lose the game not expressible.
    )
}

fn attack_double_life(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let life = script::life(state, trig.controller).max(0) as u32;
    vec![Effect::SetLifeTotal {
        player: trig.controller,
        amount: life.saturating_mul(2),
    }]
}
