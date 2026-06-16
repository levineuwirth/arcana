//! Kain, Traitorous Dragoon — `{2}{B}` 2/4 Legendary Human Knight.
//! Jump — During your turn, Kain has flying. (GAP — no Jump/conditional-keyword primitive)
//! Whenever Kain deals combat damage to a player, that player gains control of
//! Kain. If they do, you draw that many cards, create that many tapped Treasure
//! tokens, then lose that much life.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kain, Traitorous Dragoon");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Jump — During your turn, Kain has flying." — conditional keyword
    // granted only on your turn; no static-keyword-by-turn primitive available.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: kain_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "that player gains control of Kain. If they do, you draw that many cards,
/// create that many tapped Treasure tokens, then lose that much life."
fn kain_combat_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    let Some(them) = trig.damaged_player() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::ChangeControl {
        target: trig.source,
        new_controller: them,
    }];
    // "If they do, you draw that many cards, create that many tapped Treasure
    // tokens, then lose that much life." Control change always succeeds here.
    effects.push(Effect::DrawCards {
        player: trig.controller,
        count: n,
    });
    // NOTE: "tapped" Treasure tokens — CreateCommodityToken mints them but does
    // not expose a tapped flag; minted untapped (fidelity gap on the tapped rider).
    if n > 0 {
        effects.push(Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: n,
        });
    }
    effects.push(Effect::LoseLife {
        player: trig.controller,
        amount: n,
    });
    effects
}
